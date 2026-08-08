export function setupCommentForms(containerOrId = 'wiki-output', options = {}) {
    const container = typeof containerOrId === 'string' 
        ? document.getElementById(containerOrId) 
        : containerOrId;

    if (!container) return;

    container.querySelectorAll('.wiki-comment-form').forEach(form => {
        if (form.dataset.commentInitialized) return;
        form.dataset.commentInitialized = "true";

        form.addEventListener('submit', async (e) => {
            e.preventDefault();
            const nameInput = form.querySelector('input[name="name"]');
            const bodyTextarea = form.querySelector('textarea[name="body"]');
            const button = form.querySelector('.comment-submit');
            const buttonText = button ? button.querySelector('span') : null;

            if (!bodyTextarea || !bodyTextarea.value) return;

            const name = nameInput ? nameInput.value : '';
            const body = bodyTextarea.value;
            const wikiSlug = form.dataset.wikiSlug;
            const pageSlug = form.dataset.pageSlug;
            const position = form.dataset.position;

            try {
                if (button) button.disabled = true;
                if (buttonText) buttonText.textContent = "送信中...";

                const commentLine = `- ${body} -- ${name} &new{&now;};`;
                const response = await fetch(`https://asakura-wiki.vercel.app/api/wiki_v2/${wikiSlug}/${pageSlug}`, { cache: 'no-store' });

                if (!response.ok) {
                    bodyTextarea.value = '';
                    if (nameInput) nameInput.value = '';
                    return;
                }

                const pageData = await response.json();
                let content = '';
                const pakoLib = options.pako || (typeof Pako !== 'undefined' ? Pako : (window.pako ? window.pako : null));

                if (pakoLib) {
                    const binString = atob(pageData.content);
                    const compressed = Uint8Array.from(binString, (m) => m.codePointAt(0));
                    content = pakoLib.ungzip(compressed, { to: "string" });
                } else {
                    throw new Error("Pako ライブラリが指定されていないか、ロードされていません。");
                }

                const title = pageData.title;
                const commentRegex = /#comment(?:\(\s*(above|below)\s*\))?/;
                const match = content.match(commentRegex);

                if (!match) throw new Error("コンテンツ内に #comment が見つかりません");

                const index = match.index;
                const tokenLength = match[0].length;
                let updatedContent = position === 'above' ? 
                    content.slice(0, index) + commentLine + "\n" + content.slice(index) :
                    content.slice(0, index + tokenLength) + "\n" + commentLine + content.slice(index + tokenLength);

                let token = '';
                if (typeof options.getAccessToken === 'function') {
                    token = await options.getAccessToken();
                }

                const updateRes = await fetch(`https://asakura-wiki.vercel.app/api/wiki_v2/${wikiSlug}/${pageSlug}`, {
                    method: 'PUT',
                    headers: { 'Content-Type': 'application/json', ...(token ? { 'Authorization': `Bearer ${token}` } : {}) },
                    body: JSON.stringify({ title, content: updatedContent }),
                });

                if (!updateRes.ok) throw new Error('更新に失敗しました');

                bodyTextarea.value = '';
                window.location.reload();
            } catch (e) {
                console.error("#comment Submit Error: ", e);
                alert(`エラーが発生しました: ${e.message}`);
            } finally {
                if (button) button.disabled = false;
                if (buttonText) buttonText.textContent = "コメント送信";
            }
        });
    });
}

/**
 * リアルタイムコメント（#rtcomment）の初期化と通信制御を行う関数
 * Supabase へのデータ取得・送信処理はすべて Wasm (Rust) 側で完結させます。
 * * @param {string} containerId - 対象のコンテナID
 * @param {Object} [options] - オプション設定
 * @param {Object} [options.wasmModule] - wasm_bindgen でロードした Wasm モジュール
 */
export function setupRealTimeComments(containerOrId = 'wiki-output', options = {}) {
    const container = typeof containerOrId === 'string' 
        ? document.getElementById(containerOrId) 
        : containerOrId;

    if (!container) return;

    const placeholders = container.querySelectorAll('.wiki-rtcomment');
    if (placeholders.length === 0) return;

    const { wasmModule } = options;

    placeholders.forEach(async (el) => {
        if (el.dataset.rtcommentInitialized) return;
        el.dataset.rtcommentInitialized = "true";

        const wikiSlug = el.dataset.wikiSlug;
        const pageSlug = el.dataset.pageSlug;

        // 1. DOM構築（コメント一覧とフォーム）
        el.innerHTML = `
            <div style="margin-top: 1em;">
                <ul class="rtcomment-list" style="margin-bottom: 1em; list-style: none; padding-left: 0;"></ul>
                <form class="rtcomment-form">
                    <div style="margin-bottom: 0.5em;">
                        <input type="text" name="name" placeholder="名前" style="padding: 0.5em; width: 100%; margin-bottom: 0.5em;" />
                        <textarea name="body" placeholder="コメント" rows="3" style="padding: 0.5em; width: 100%;" required></textarea>
                    </div>
                    <button type="submit" class="comment-submit" style="padding: 0.5em 1em; background-color: #ea94bc; color: white; border: none; cursor: pointer;">
                        <span>送信</span>
                    </button>
                </form>
            </div>
        `;

        const listEl = el.querySelector('.rtcomment-list');
        const formEl = el.querySelector('.rtcomment-form');
        const submitBtn = el.querySelector('.comment-submit');
        const submitBtnText = submitBtn ? submitBtn.querySelector('span') : null;

        // コメント要素を DOM に追加するヘルパー関数
        const renderComment = (c) => {
            if (c.id && listEl.querySelector(`[data-comment-id="${c.id}"]`)) return;

            const li = document.createElement('li');
            if (c.id) li.setAttribute('data-comment-id', c.id);
            li.style.marginBottom = '0.5em';

            const strong = document.createElement('strong');
            strong.textContent = c.name || '名無し';

            const dateStr = c.created_at ? new Date(c.created_at).toLocaleString() : new Date().toLocaleString();
            const timeSpan = document.createTextNode(` (${dateStr}):`);

            const br = document.createElement('br');

            const p = document.createElement('p');
            p.style.margin = '0';
            p.style.whiteSpace = 'pre-wrap';
            p.textContent = c.body;

            li.appendChild(strong);
            li.appendChild(timeSpan);
            li.appendChild(br);
            li.appendChild(p);
            listEl.appendChild(li);
        };

        // 2. Rust (Wasm) 経由での過去コメント取得
        async function loadComments() {
            if (wasmModule && typeof wasmModule.fetch_comments_wasm === 'function') {
                try {
                    // Rust 内で SUPABASE_URL と SUPABASE_KEY を参照するため、slug のみを渡す
                    const jsonText = await wasmModule.fetch_comments_wasm(wikiSlug, pageSlug);
                    const comments = JSON.parse(jsonText);
                    if (Array.isArray(comments)) {
                        listEl.innerHTML = '';
                        comments.forEach(c => renderComment(c));
                    }
                } catch (err) {
                    console.error('[Wasm fetch_comments_wasm error]', err);
                }
            }
        }

        await loadComments();

        // 3. Rust (Wasm) 経由でのコメント送信処理
        formEl.addEventListener('submit', async (e) => {
            e.preventDefault();
            const nameInput = formEl.querySelector('input[name="name"]');
            const bodyTextarea = formEl.querySelector('textarea[name="body"]');

            const name = nameInput.value;
            const body = bodyTextarea.value;

            if (!name.trim() || !body.trim()) {
                alert('名前とコメントを入力してください');
                return;
            }

            try {
                if (submitBtn) submitBtn.disabled = true;
                if (submitBtnText) submitBtnText.textContent = '送信中...';

                if (wasmModule && typeof wasmModule.send_comment_wasm === 'function') {
                    const ok = await wasmModule.send_comment_wasm(
                        wikiSlug,
                        pageSlug,
                        name,
                        body,
                        null
                    );

                    if (ok) {
                        bodyTextarea.value = '';
                        // 送信成功時に再取得して反映
                        await loadComments();
                    } else {
                        throw new Error('Wasm 側での送信処理に失敗しました');
                    }
                } else {
                    console.warn('Wasm モジュールが設定されていないため、送信できません');
                }
            } catch (err) {
                alert('送信に失敗しました');
                console.error('送信エラー:', err);
            } finally {
                if (submitBtn) submitBtn.disabled = false;
                if (submitBtnText) submitBtnText.textContent = '送信';
            }
        });
    });
}
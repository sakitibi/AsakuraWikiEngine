import { setupCommentForms, setupRealTimeComments } from "./comment.js";

export function applyMarqueeResponsive(containerId = 'wiki-output') {
    const container = document.getElementById(containerId);
    if (!container) return;

    let sizeSuffix = 'xl';
    const screenWidth = window.innerWidth;
    if (screenWidth < 700) sizeSuffix = 'sm';
    else if (screenWidth < 1000) sizeSuffix = 'md';

    if (sizeSuffix !== 'xl') {
        container.querySelectorAll('[style*="animation-name: scroll-"]').forEach(el => {
            el.style.animationName = el.style.animationName.replace('-xl', `-${sizeSuffix}`);
        });
    }
}

/**
 * HTMLとしてレンダリングされた #accordion のインタラクティブ動作をセットアップする関数
 * @param {string|HTMLElement} containerOrId - 対象のコンテナIDまたはDOM要素
 */
export function setupAccordions(containerOrId = 'wiki-output') {
    const container = typeof containerOrId === 'string' 
        ? document.getElementById(containerOrId) 
        : containerOrId;
    if (!container) return;

    container.querySelectorAll('.accordion-header').forEach(header => {
        if (header.dataset.accordionInitialized) return;
        header.dataset.accordionInitialized = "true";

        header.addEventListener('click', () => {
            const content = header.nextElementSibling;
            if (!content || !content.classList.contains('accordion-content')) return;

            // クラスの着脱で開閉状態を管理
            const isOpen = content.classList.toggle('is-open');

            // アイコンのSVGパスを切り替える
            const iconPath = header.querySelector('svg path');
            if (iconPath) {
                if (isOpen) {
                    // 開いた時のアイコンパス
                    iconPath.setAttribute('d', 'M384 32H64C28.7 32 0 60.7 0 96v320c0 35.3 28.7 64 64 64h320c35.3 0 64-28.7 64-64V96c0-35.3-28.7-64-64-64zM320 272H128c-13.3 0-24-10.7-24-24s10.7-24 24-24h192c13.3 0 24 10.7 24 24s-10.7 24-24 24z');
                } else {
                    // 閉じている時のアイコンパス
                    iconPath.setAttribute('d', 'M64 32C28.7 32 0 60.7 0 96L0 416c0 35.3 28.7 64 64 64l320 0c35.3 0 64-28.7 64-64l0-320c0-35.3-28.7-64-64-64L64 32zM200 344l0-64-64 0c-13.3 0-24-10.7-24-24s10.7-24 24-24l64 0 0-64c0-13.3 10.7-24 24-24s24 10.7 24 24l0 64 64 0c13.3 0 24 10.7 24 24s-10.7 24-24 24l-64 0 0 64c0 13.3-10.7 24-24 24s-24-10.7-24-24z');
                }
            }
        });
    });
}

/**
 * .wiki-include-page プレースホルダーを検出し、非同期で対象ページを取得してレンダリングする関数
 * @param {string} containerId - 対象のコンテナID
 * @param {object} wasmModule - parseWiki 関数を保持するWasmモジュール
 * @param {string} currentWikiSlug - 現在のWikiスラグ
 */
/**
 * .wiki-include-page プレースホルダーを検出し、非同期で対象ページを取得してレンダリングする関数
 * @param {string|HTMLElement} containerOrId - 対象のコンテナIDまたはDOM要素
 * @param {object} wasmModule - parseWiki を含むWasmモジュール
 * @param {string} currentWikiSlug - 現在のWikiスラグ
 * @param {object} extraHandlers - 子ページ初期化用ハンドラ群
 */
export async function setupIncludePages(containerOrId = 'wiki-output', wasmModule, currentWikiSlug, extraHandlers = {}) {
    const container = typeof containerOrId === 'string' 
        ? document.getElementById(containerOrId) 
        : containerOrId;
    if (!container) return;

    // この階層にあるプレースホルダーのみを処理
    const placeholders = container.querySelectorAll('.wiki-include-page');

    for (const el of placeholders) {
        if (el.dataset.includeInitialized) continue;
        el.dataset.includeInitialized = "true";

        const page = el.dataset.page;
        const stylesheet = el.dataset.stylesheet;
        const range = el.dataset.range;
        const showTitle = el.dataset.showTitle;

        if (stylesheet) {
            const link = document.createElement('link');
            link.rel = 'stylesheet';
            link.href = stylesheet;
            document.head.appendChild(link);
        }

        try {
            const res = await fetch(`https://asakura-wiki.vercel.app/api/wiki_v2/${currentWikiSlug}/${encodeURIComponent(page)}`);
            if (!res.ok) throw new Error(`${res.status} ${res.statusText}`);

            const data = await res.json();
            const contentStr = data.content ?? '';

            if (!contentStr) {
                el.innerHTML = '';
                continue;
            }

            const binaryString = atob(contentStr);
            const len = binaryString.length;
            const uint8Array = new Uint8Array(len);
            for (let idx = 0; idx < len; idx++) {
                uint8Array[idx] = binaryString.charCodeAt(idx);
            }

            const pakoInstance = window.pako || (typeof pako !== 'undefined' ? pako : null);
            if (!pakoInstance) throw new Error("pako library is not loaded");
            
            let text = pakoInstance.ungzip(uint8Array, { to: 'string' });

            if (range) {
                const parts = range.split('-');
                const lines = text.split('\n');

                let start = parts[0] ? parseInt(parts[0], 10) : 1;
                let end = parts[1] ? parseInt(parts[1], 10) : lines.length;

                if (end > lines.length) {
                    end = lines.length;
                }

                if (!isNaN(start) && !isNaN(end) && start >= 1 && start <= end) {
                    text = lines.slice(start - 1, end).join('\n');
                } else {
                    el.innerHTML = '<p style="color: red;">読み込み失敗: 無効な行範囲です</p>';
                    continue;
                }
            }

            if (!wasmModule || typeof wasmModule.parseWiki !== 'function') {
                throw new Error("wasmModule.parseWiki is not a function");
            }
            const parsedHtml = wasmModule.parseWiki(text, currentWikiSlug, page);

            const shouldShowTitle = showTitle !== 'false';
            
            let finalHtml = '';
            if (shouldShowTitle) {
                finalHtml += `<h2 class="include-page__title">${page}</h2>`;
            }
            finalHtml += parsedHtml;

            // HTMLの流し込み
            el.innerHTML = finalHtml;

            setupAccordions(el); // 子ページ内のアコーディオン対応
            setupPageLists(el);

            if (typeof extraHandlers.setupCommentForms === 'function') {
                extraHandlers.setupCommentForms(el);
            }
            if (typeof extraHandlers.setupRealTimeComments === 'function') {
                // rtcommentにはWasmモジュールのリレーが必要なため引数を合わせる
                extraHandlers.setupRealTimeComments(el, { wasmModule });
            }

            // 子ページの中にさらに #include があった場合の再帰処理
            await setupIncludePages(el, wasmModule, currentWikiSlug, extraHandlers);

        } catch (err) {
            console.error('Include page error:', err);
            el.innerHTML = `<p style="color: red;">読み込み失敗: ${err.message}</p>`;
        }
    }
}

/**
 * .wiki-pagelist および .wiki-pagelist2 プレースホルダーを検出し、
 * 非同期で対象Wikiのページ一覧を取得してレンダリングする関数
 * @param {string|HTMLElement} containerOrId - 対象のコンテナIDまたはDOM要素
 */
export async function setupPageLists(containerOrId = 'wiki-output') {
    const container = typeof containerOrId === 'string' 
        ? document.getElementById(containerOrId) 
        : containerOrId;
    if (!container) return;

    // #ls (#ls2) のプレースホルダー要素を両方取得
    const lists = container.querySelectorAll('.wiki-pagelist, .wiki-pagelist2');

    for (const el of lists) {
        if (el.dataset.pagelistInitialized) continue;
        el.dataset.pagelistInitialized = "true";

        const wikiSlug = el.dataset.wikiSlug;
        const prefix = el.dataset.prefix || "";       // #ls 用
        const pattern = el.dataset.pattern || "";     // #ls2 用
        const options = el.dataset.options || "";     // #ls2 用
        const label = el.dataset.label || "";         // #ls2 用

        try {
            const res = await fetch(`https://asakura-wiki.vercel.app/api/wiki_v2/${wikiSlug}`);
            if (!res.ok) throw new Error(`${res.status} ${res.statusText}`);

            const data = await res.json();
            const pages = data.page_slugs;
            
            if (!Array.isArray(pages)) {
                throw new Error("ページ一覧データが配列ではありません");
            }

            // フィルタリング処理
            let filteredPages = pages;
            const isLs2 = el.classList.contains('wiki-pagelist2');

            if (isLs2) {
                // #ls2 のパターンマッチング (前方一致など、仕様に合わせて調整)
                if (pattern) {
                    filteredPages = pages.filter(p => p.startsWith(pattern));
                }
            } else {
                // #ls の接頭辞マッチング
                if (prefix) {
                    filteredPages = pages.filter(p => p.startsWith(prefix));
                }
            }

            if (filteredPages.length === 0) {
                el.innerHTML = '<ul class="wiki-page-list-ul"><li>(該当するページはありません)</li></ul>';
                continue;
            }

            // HTMLリストの構築
            let html = '<ul class="wiki-page-list-ul">';
            for (const pageName of filteredPages) {
                let displayLabel = pageName;
                
                html += `<li><a href="#" data-wiki-link="${pageName}" class="wiki-link">${displayLabel}</a></li>`;
            }
            html += '</ul>';

            el.innerHTML = html;

        } catch (err) {
            console.error('PageList load error:', err);
            el.innerHTML = `<p style="color: red;">リスト読み込み失敗: ${err.message}</p>`;
        }
    }
}
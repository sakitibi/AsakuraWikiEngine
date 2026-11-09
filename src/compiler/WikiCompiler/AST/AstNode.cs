namespace WikiCompiler.AST
{
    public interface IAstNode
    {
        string ToHtml();
    }

    // 通常テキストノード
    public class TextNode : IAstNode
    {
        public string Text { get; set; } = string.Empty;

        public string ToHtml() => $"{Text}";
    }
}
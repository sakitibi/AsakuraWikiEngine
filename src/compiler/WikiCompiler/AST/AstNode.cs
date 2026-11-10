namespace WikiCompiler.AST
{
    public interface IAstNode
    {}

    // 通常テキストノード
    public class TextNode : IAstNode
    {
        public string Text { get; set; } = string.Empty;
    }
    public class InlinePluginNode : IAstNode
    {
        public string PluginName { get; set; } = string.Empty;
        public string Argument { get; set; } = string.Empty;
        public string Text { get; set; } = string.Empty;
    }
    public class ColorPluginNode : IAstNode
    {
        public string Color { get; set; } = string.Empty;
        public string BackgroundColor { get; set; } = string.Empty;
        public string Text { get; set; } = string.Empty;
    }

    public class SizePluginNode : IAstNode
    {
        public string Size { get; set; } = string.Empty;
        public string Text { get; set; } = string.Empty;
    }
}
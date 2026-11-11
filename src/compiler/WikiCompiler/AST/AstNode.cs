namespace WikiCompiler.AST
{
    public interface IAstNode { }

    public class TextNode : IAstNode
    {
        public string Text { get; set; } = string.Empty;
    }

    public class ColorPluginNode : IAstNode
    {
        public string Color { get; set; } = string.Empty;
        public string BackgroundColor { get; set; } = string.Empty;
        public IAstNode InnerNode { get; set; } = null!;
    }

    public class SizePluginNode : IAstNode
    {
        public string Size { get; set; } = string.Empty;
        public IAstNode InnerNode { get; set; } = null!;
    }

    public class HeadingNode : IAstNode
    {
        public int Level { get; set; }
        public string Anchor { get; set; } = string.Empty;
        public IAstNode InnerNode { get; set; } = null!;
    }
}
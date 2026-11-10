namespace WikiCompiler.AST
{
    public enum AlignType
    {
        Left,
        Center,
        Right
    }

    public class AlignNode : IAstNode
    {
        public AlignType Align { get; set; }
        public IAstNode InnerNode { get; set; } = null!;
    }
}
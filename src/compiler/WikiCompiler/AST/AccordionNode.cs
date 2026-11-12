namespace WikiCompiler.AST
{
    public class AccordionNode : IAstNode
    {
        public int BraceLevel { get; set; }
        public IAstNode TitleNode { get; set; } = null!;
        public int HeadingLevel { get; set; }
        public bool IsOpen { get; set; } = true;
        public List<IAstNode> Children { get; set; } = [];
    }
}
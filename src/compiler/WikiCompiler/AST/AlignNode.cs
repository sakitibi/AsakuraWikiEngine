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
        public string Text { get; set; } = string.Empty;

        public string ToHtml()
        {
            string alignStr = Align.ToString().ToLower();
            return $"<div style=\"text-align: {alignStr};\">{Text}</div>";
        }
    }
}
using WikiCompiler.AST;

namespace WikiCompiler.Parser
{
    public class WikiParser
    {
        public List<IAstNode> Parse(string[] lines)
        {
            List<IAstNode> nodes = [];

            foreach (string line in lines)
            {
                if (string.IsNullOrWhiteSpace(line)) continue;

                string trimmed = line.Trim();

                if (trimmed.StartsWith("CENTER:", StringComparison.OrdinalIgnoreCase))
                {
                    nodes.Add(new AlignNode
                    {
                        Align = AlignType.Center,
                        Text = trimmed[7..].Trim()
                    });
                }
                else if (trimmed.StartsWith("RIGHT:", StringComparison.OrdinalIgnoreCase))
                {
                    nodes.Add(new AlignNode
                    {
                        Align = AlignType.Right,
                        Text = trimmed[6..].Trim()
                    });
                }
                else if (trimmed.StartsWith("LEFT:", StringComparison.OrdinalIgnoreCase))
                {
                    nodes.Add(new AlignNode
                    {
                        Align = AlignType.Left,
                        Text = trimmed[5..].Trim()
                    });
                }
                else
                {
                    // 通常テキストのパース
                    nodes.Add(new TextNode
                    {
                        Text = trimmed
                    });
                }
            }

            return nodes;
        }
    }
}
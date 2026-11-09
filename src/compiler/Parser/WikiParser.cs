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

                // 通常テキストのパース
                nodes.Add(new TextNode
                {
                    Text = line.Trim()
                });
            }

            return nodes;
        }
    }
}
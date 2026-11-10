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

                // 1. 行頭の配置指定（AlignNode）を判定（左端固定）
                AlignType? alignType = null;
                if (trimmed.StartsWith("CENTER:", StringComparison.OrdinalIgnoreCase))
                {
                    alignType = AlignType.Center;
                    trimmed = trimmed["CENTER:".Length..].Trim();
                }
                else if (trimmed.StartsWith("RIGHT:", StringComparison.OrdinalIgnoreCase))
                {
                    alignType = AlignType.Right;
                    trimmed = trimmed["RIGHT:".Length..].Trim();
                }
                else if (trimmed.StartsWith("LEFT:", StringComparison.OrdinalIgnoreCase))
                {
                    alignType = AlignType.Left;
                    trimmed = trimmed["LEFT:".Length..].Trim();
                }

                IAstNode innerNode = ParseInlineRecursive(trimmed);

                if (alignType.HasValue)
                {
                    nodes.Add(new AlignNode
                    {
                        Align = alignType.Value,
                        InnerNode = innerNode
                    });
                }
                else
                {
                    nodes.Add(innerNode);
                }
            }

            return nodes;
        }

        private static IAstNode ParseInlineRecursive(string input)
        {
            input = input.Trim();

            if (input.StartsWith('&'))
            {
                int parenStart = input.IndexOf('(', 0);
                if (parenStart != -1)
                {
                    string pluginName = input[1..parenStart].ToLower();
                    if (pluginName == "color" || pluginName == "size")
                    {
                        int parenEnd = input.IndexOf(')', parenStart);
                        if (parenEnd != -1 && parenEnd + 1 < input.Length && input[parenEnd + 1] == '{')
                        {
                            string argsPart = input[(parenStart + 1)..parenEnd];
                            int contentStart = parenEnd + 2;
                            int braceDepth = 1;
                            int contentEnd = -1;

                            for (int j = contentStart; j < input.Length; j++)
                            {
                                if (input[j] == '{')
                                {
                                    braceDepth++;
                                }
                                else if (input[j] == '}')
                                {
                                    braceDepth--;
                                    if (braceDepth == 0)
                                    {
                                        contentEnd = j;
                                        break;
                                    }
                                }
                            }

                            if (contentEnd != -1 && contentEnd + 1 < input.Length && input[contentEnd + 1] == ';')
                            {
                                string innerContent = input[contentStart..contentEnd];
                                IAstNode nestedNode = ParseInlineRecursive(innerContent);

                                if (pluginName == "color")
                                {
                                    string color = argsPart;
                                    string bgColor = string.Empty;
                                    if (argsPart.Contains(','))
                                    {
                                        var parts = argsPart.Split([','], 2);
                                        color = parts[0].Trim();
                                        bgColor = parts[1].Trim();
                                    }
                                    return new ColorPluginNode
                                    {
                                        Color = color,
                                        BackgroundColor = bgColor,
                                        InnerNode = nestedNode
                                    };
                                }
                                else if (pluginName == "size")
                                {
                                    return new SizePluginNode
                                    {
                                        Size = argsPart,
                                        InnerNode = nestedNode
                                    };
                                }
                            }
                        }
                    }
                }
            }

            return new TextNode { Text = input };
        }
    }
}
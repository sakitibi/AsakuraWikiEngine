using System.Text.RegularExpressions;
using WikiCompiler.AST;
using WikiCompiler.Parser.Plugins;

namespace WikiCompiler.Parser
{
    public partial class WikiParser
    {
        [GeneratedRegex(@"\[#([a-zA-Z0-9_-]+)\]\s*$")]
        private static partial Regex AnchorRegex();

        public List<IAstNode> Parse(string[] lines)
        {
            int index = 0;
            return ParseBlock(lines, ref index, 0);
        }

        internal static List<IAstNode> ParseBlock(string[] lines, ref int index, int targetBraceLevel)
        {
            List<IAstNode> nodes = [];

            while (index < lines.Length)
            {
                string line = lines[index];
                if (string.IsNullOrWhiteSpace(line))
                {
                    index++;
                    continue;
                }
                string trimmed = line.Trim();

                // アコーディオンの終了判定
                if (targetBraceLevel > 0)
                {
                    string closingBraces = new('}', targetBraceLevel);
                    if (trimmed.StartsWith(closingBraces, StringComparison.Ordinal))
                    {
                        index++;
                        break;
                    }
                }

                if (Accordion.TryParseAccordion(lines, ref index, targetBraceLevel, out var accordionNode))
                {
                    nodes.Add(accordionNode);
                    continue;
                }

                nodes.Add(ParseLine(trimmed));
                index++;
            }

            return nodes;
        }

        private static IAstNode ParseLine(string trimmed)
        {
            int headingLevel = 0;
            if (trimmed.StartsWith("***", StringComparison.Ordinal))
            {
                headingLevel = 3;
                trimmed = trimmed[3..].Trim();
            }
            else if (trimmed.StartsWith("**", StringComparison.Ordinal))
            {
                headingLevel = 2;
                trimmed = trimmed[2..].Trim();
            }
            else if (trimmed.Length > 0 && trimmed[0] == '*')
            {
                headingLevel = 1;
                trimmed = trimmed[1..].Trim();
            }

            string anchorName = string.Empty;
            var anchorMatch = AnchorRegex().Match(trimmed);
            if (anchorMatch.Success)
            {
                anchorName = anchorMatch.Groups[1].Value;
                trimmed = trimmed[..anchorMatch.Index].Trim();
            }

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

            IAstNode finalNode = innerNode;
            if (alignType.HasValue)
            {
                finalNode = new AlignNode
                {
                    Align = alignType.Value,
                    InnerNode = finalNode
                };
            }

            if (headingLevel > 0)
            {
                return new HeadingNode
                {
                    Level = headingLevel,
                    Anchor = anchorName,
                    InnerNode = finalNode
                };
            }

            return finalNode;
        }

        internal static IAstNode ParseInlineRecursive(string input)
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
                                if (input[j] == '{') braceDepth++;
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
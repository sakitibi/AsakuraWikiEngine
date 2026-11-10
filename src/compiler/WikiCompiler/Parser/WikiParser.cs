using System.Text.RegularExpressions;
using WikiCompiler.AST;

namespace WikiCompiler.Parser
{
    public class WikiParser
    {
        private static readonly Regex PluginRegex = new(@"&([a-zA-Z]+)\(([^)]+)\)\{(.+?)\};");

        public List<IAstNode> Parse(string[] lines)
        {
            List<IAstNode> nodes = [];

            foreach (string line in lines)
            {
                if (string.IsNullOrWhiteSpace(line)) continue;
                string trimmed = line.Trim();

                if (trimmed.StartsWith("CENTER:", StringComparison.OrdinalIgnoreCase))
                {
                    string textContent = trimmed["CENTER:".Length..].Trim();
                    nodes.Add(new AlignNode
                    {
                        Align = AlignType.Center,
                        Text = textContent
                    });
                    continue;
                }
                else if (trimmed.StartsWith("RIGHT:", StringComparison.OrdinalIgnoreCase))
                {
                    string textContent = trimmed["RIGHT:".Length..].Trim();
                    nodes.Add(new AlignNode
                    {
                        Align = AlignType.Right,
                        Text = textContent
                    });
                    continue;
                }
                else if (trimmed.StartsWith("LEFT:", StringComparison.OrdinalIgnoreCase))
                {
                    string textContent = trimmed["LEFT:".Length..].Trim();
                    nodes.Add(new AlignNode
                    {
                        Align = AlignType.Left,
                        Text = textContent
                    });
                    continue;
                }

                var match = PluginRegex.Match(trimmed);
                if (match.Success)
                {
                    string pluginName = match.Groups[1].Value.ToLower();
                    string argsPart = match.Groups[2].Value;
                    string text = match.Groups[3].Value;

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

                        nodes.Add(new ColorPluginNode
                        {
                            Color = color,
                            BackgroundColor = bgColor,
                            Text = text
                        });
                        continue;
                    }
                    else if (pluginName == "size")
                    {
                        nodes.Add(new SizePluginNode
                        {
                            Size = argsPart,
                            Text = text
                        });
                        continue;
                    }
                }

                nodes.Add(new TextNode { Text = trimmed });
            }

            return nodes;
        }
    }
}
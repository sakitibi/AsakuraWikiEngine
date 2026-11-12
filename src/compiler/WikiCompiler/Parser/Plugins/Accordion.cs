using System.Text.RegularExpressions;
using WikiCompiler.AST;

namespace WikiCompiler.Parser.Plugins
{
    public partial class Accordion
    {
        [GeneratedRegex(@"^#accordion\((.*)\)(\{\{+)\s*$")]
        private static partial Regex AccordionStartRegex();

        internal static bool TryParseAccordion(string[] lines, ref int index, int targetBraceLevel, out IAstNode resultNode)
        {
            resultNode = null!;
            string trimmed = lines[index].Trim();
            var accordionMatch = AccordionStartRegex().Match(trimmed);

            if (!accordionMatch.Success)
            {
                return false;
            }

            string rawArgs = accordionMatch.Groups[1].Value;
            string braces = accordionMatch.Groups[2].Value;
            int braceLevel = braces.Length;

            List<string> args = SplitTopLevelArgs(rawArgs);

            string rawTitle = args.Count > 0 ? args[0].Trim() : string.Empty;
            IAstNode titleNode = WikiParser.ParseInlineRecursive(rawTitle);

            int headingLevel = 0;
            if (args.Count > 1)
            {
                string levelArg = args[1].Trim();
                headingLevel = levelArg switch
                {
                    "*" => 1,
                    "**" => 2,
                    "***" => 3,
                    _ => 0
                };
            }

            bool isOpen = true;
            if (args.Count > 2)
            {
                string stateArg = args[2].Trim();
                if (stateArg.Equals("close", StringComparison.OrdinalIgnoreCase))
                {
                    isOpen = false;
                }
                else if (stateArg.Equals("open", StringComparison.OrdinalIgnoreCase))
                {
                    isOpen = true;
                }
            }

            index++;
            List<IAstNode> innerNodes = WikiParser.ParseBlock(lines, ref index, braceLevel);

            resultNode = new AccordionNode
            {
                BraceLevel = braceLevel,
                TitleNode = titleNode,
                HeadingLevel = headingLevel,
                IsOpen = isOpen,
                Children = innerNodes
            };

            return true;
        }

        private static List<string> SplitTopLevelArgs(string input)
        {
            List<string> result = [];
            int parenDepth = 0;
            int braceDepth = 0;
            int start = 0;

            for (int i = 0; i < input.Length; i++)
            {
                char c = input[i];
                if (c == '(') parenDepth++;
                else if (c == ')') parenDepth--;
                else if (c == '{') braceDepth++;
                else if (c == '}') braceDepth--;
                else if (c == ',' && parenDepth == 0 && braceDepth == 0)
                {
                    result.Add(input[start..i]);
                    start = i + 1;
                }
            }
            result.Add(input[start..]);
            return result;
        }
    }
}
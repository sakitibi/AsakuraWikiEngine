using WikiCompiler.Parser;

namespace WikiCompiler
{
    class Program
    {
        static void Main(string[] args)
        {
            if (args.Length < 1)
            {
                Console.WriteLine("Usage: wikic <input.wiki>");
                return;
            }

            string inputPath = args[0];
            if (!File.Exists(inputPath))
            {
                Console.WriteLine($"Error: File not found -> {inputPath}");
                return;
            }

            string[] lines = File.ReadAllLines(inputPath);
            
            // 構文解析の実行
            WikiParser parser = new();
            List<AST.IAstNode> astNodes = parser.Parse(lines);

            Console.WriteLine($"Parsed {astNodes.Count} nodes successfully.");
        }
    }
}
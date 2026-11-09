using WikiCompiler.CodeGen;
using WikiCompiler.Parser;

namespace WikiCompiler
{
    class Program
    {
        static void Main(string[] args)
        {
            if (args.Length < 1)
            {
                Console.WriteLine("Usage: wikic <input.askr> [output.askb]");
                return;
            }

            string inputPath = args[0];
            string outputPath = args.Length > 1 ? args[1] : Path.ChangeExtension(inputPath, ".askb");

            if (!File.Exists(inputPath))
            {
                Console.WriteLine($"Error: File not found -> {inputPath}");
                return;
            }

            string[] lines = File.ReadAllLines(inputPath);
            
            WikiParser parser = new();
            List<AST.IAstNode> astNodes = parser.Parse(lines);

            Console.WriteLine($"Parsed {astNodes.Count} nodes successfully.");

            BytecodeEmitter.Emit(astNodes, outputPath);

            Console.WriteLine($"Successfully emitted compiled binary to: {outputPath}");
        }
    }
}
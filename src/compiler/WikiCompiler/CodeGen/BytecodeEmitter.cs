using System.Text;
using WikiCompiler.AST;

namespace WikiCompiler.CodeGen
{
    public class BytecodeEmitter
    {
        public static void Emit(List<IAstNode> nodes, string outputPath)
        {
            using BinaryWriter writer = new(File.Open(outputPath, FileMode.Create), Encoding.UTF8);

            foreach (IAstNode node in nodes)
            {
                switch (node)
                {
                    case AlignNode alignNode:
                        writer.Write(GetOpCodeForAlign(alignNode.Align));
                        writer.Write(alignNode.Text);
                        break;

                    case TextNode textNode:
                        writer.Write((byte)0x02);
                        writer.Write(textNode.Text);
                        break;
                }
            }
        }

        private static byte GetOpCodeForAlign(AlignType align) => align switch
        {
            AlignType.Center => 0x04,
            AlignType.Right  => 0x05,
            AlignType.Left   => 0x06,
            _ => 0x02
        };
    }
}
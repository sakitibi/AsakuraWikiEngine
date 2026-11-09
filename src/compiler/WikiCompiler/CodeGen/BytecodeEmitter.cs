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
                    case TextNode textNode:
                        writer.Write((byte)0x02);
                        WriteLengthAndString(writer, textNode.Text);
                        writer.Write((byte)0x03);
                        break;

                    case AlignNode alignNode:
                        writer.Write(GetOpCodeForAlign(alignNode.Align));
                        writer.Write((byte)0x02);
                        WriteLengthAndString(writer, alignNode.Text);
                        writer.Write((byte)0x03);
                        break;
                }
            }
        }

        private static void WriteLengthAndString(BinaryWriter writer, string text)
        {
            byte[] utf8Bytes = Encoding.UTF8.GetBytes(text);
            
            if (utf8Bytes.Length > 255)
            {
                throw new InvalidOperationException("One line is too long (max 255 bytes in UTF-8).");
            }

            writer.Write((byte)utf8Bytes.Length);
            writer.Write(utf8Bytes);
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
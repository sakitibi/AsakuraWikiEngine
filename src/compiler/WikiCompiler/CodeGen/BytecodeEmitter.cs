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
                EmitNode(writer, node);
            }
        }

        private static void EmitNode(BinaryWriter writer, IAstNode node)
        {
            switch (node)
            {
                case HeadingNode headingNode:
                    writer.Write(GetOpCodeForHeading(headingNode.Level));
                    WriteLengthAndString(writer, headingNode.Anchor);
                    EmitNode(writer, headingNode.InnerNode);
                    break;
                
                case AlignNode alignNode:
                    writer.Write(GetOpCodeForAlign(alignNode.Align));
                    EmitNode(writer, alignNode.InnerNode);
                    break;

                case ColorPluginNode colorNode:
                    writer.Write((byte)0x07);
                    WriteLengthAndString(writer, colorNode.Color);
                    WriteLengthAndString(writer, colorNode.BackgroundColor);
                    EmitNode(writer, colorNode.InnerNode);
                    break;

                case SizePluginNode sizeNode:
                    writer.Write((byte)0x08);
                    WriteLengthAndString(writer, sizeNode.Size);
                    EmitNode(writer, sizeNode.InnerNode);
                    break;

                case AccordionNode accordionNode:
                    writer.Write((byte)0x0C);
                    writer.Write((byte)accordionNode.BraceLevel);
                    writer.Write((byte)accordionNode.HeadingLevel);
                    writer.Write((byte)(accordionNode.IsOpen ? 1 : 0));
                    EmitNode(writer, accordionNode.TitleNode);
                    writer.Write((byte)accordionNode.Children.Count);
                    foreach (var child in accordionNode.Children)
                    {
                        EmitNode(writer, child);
                    }
                    break;

                case TextNode textNode:
                    writer.Write((byte)0x02);
                    WriteLengthAndString(writer, textNode.Text);
                    writer.Write((byte)0x03);
                    break;
            }
        }

        private static void WriteLengthAndString(BinaryWriter writer, string text)
        {
            text ??= "";
            byte[] utf8Bytes = Encoding.UTF8.GetBytes(text);
            if (utf8Bytes.Length > 255)
            {
                throw new InvalidOperationException("One line is too long (max 255 bytes in UTF-8).");
            }
            writer.Write((byte)utf8Bytes.Length);
            writer.Write(utf8Bytes);
        }

        private static byte GetOpCodeForHeading(int level) => level switch
        {
            1 => 0x09,
            2 => 0x0A,
            3 => 0x0B,
            _ => 0x09
        };

        private static byte GetOpCodeForAlign(AlignType align) => align switch
        {
            AlignType.Center => 0x04,
            AlignType.Right  => 0x05,
            AlignType.Left   => 0x06,
            _ => 0x02
        };
    }
}
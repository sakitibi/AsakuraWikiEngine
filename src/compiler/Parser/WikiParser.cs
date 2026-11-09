using System;
using System.Collections.Generic;
using System.IO;
using WikiCompiler.AST;

namespace WikiCompiler.Parser
{
    public class WikiParser
    {
        public List<IAstNode> Parse(string[] lines)
        {
            var nodes = new List<IAstNode>();

            foreach (var line in lines)
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
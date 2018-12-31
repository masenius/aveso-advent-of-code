using System;
using System.IO;
using System.Collections.Generic;

namespace Day5
{
    public class Polymer
    {
        private delegate bool Filter(char c);

        private Stack<char> Chars;

        public Polymer(TextReader reader)
        {
            BuildAndReact(reader, c => true);
        }

        public Polymer(TextReader reader, char filterChar)
        {
            char upperFilterChar = Char.ToUpper(filterChar);
            BuildAndReact(reader, c => (c != filterChar && c != upperFilterChar));
        }

        public override string ToString()
        {
            // Need to reverse the stack
            return string.Concat(new Stack<char>(this.Chars));
        }

        private void BuildAndReact(TextReader reader, Filter filter)
        {
            this.Chars = new Stack<char>();

            int c;
            while ((c = reader.Read()) != -1)
            {
                var cChar = (char) c;
                if (!filter(cChar))
                {
                    continue;
                }

                if (this.Chars.Count > 0 && UnitsReact(cChar, this.Chars.Peek()))
                {
                    this.Chars.Pop();
                }
                else
                {
                    this.Chars.Push(cChar);
                }
            }
        }

        private bool UnitsReact(char u1, char u2)
        {
            if (Char.IsUpper(u1))
            {
                return Char.ToLower(u1) == u2;
            }
            else
            {
                return Char.ToUpper(u1) == u2;
            }
        }
    }
}

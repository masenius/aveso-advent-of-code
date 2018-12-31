using System;
using System.IO;
using System.Collections.Generic;

namespace Day5
{
    public class Polymer
    {
        private Stack<char> Chars;

        public Polymer(TextReader reader)
        {
            this.Chars = new Stack<char>();

            int c;
            while ((c = reader.Read()) != -1)
            {
                var cChar = (char) c;
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

        public override string ToString()
        {
            // Need to reverse the stack
            return string.Concat(new Stack<char>(this.Chars));
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

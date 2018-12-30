using System;
using System.IO;
using System.Collections.Generic;

namespace Day5
{
    public class Polymer
    {
        private LinkedList<char> CharList;

        public Polymer(TextReader reader)
        {
            this.CharList = new LinkedList<char>();

            int c;
            while ((c = reader.Read()) != -1)
            {
                this.CharList.AddLast((char) c);
            }
        }

        public void React()
        {
            var current = this.CharList.First;
            var next = current.Next;
            while (next != null)
            {
                if (UnitsReact(current.Value, next.Value))
                {
                    this.CharList.Remove(current);
                    this.CharList.Remove(next);

                    current = current.Previous;
                    if (current == null)
                    {
                        // We have removed the previously first element of the list
                        current = this.CharList.First;
                    }
                }
                else
                {
                    current = next;
                }
                next = current.Next;
            }
        }

        public override string ToString()
        {
            return string.Concat(this.CharList);
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

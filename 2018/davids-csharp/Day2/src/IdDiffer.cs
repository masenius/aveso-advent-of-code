using System;
using System.Collections.Generic;

namespace Day2
{
    public class IDDiffer
    {
        public static bool DifferByOne(string id1, string id2, out string letters)
        {
            letters = null;
            if (id1.Length != id2.Length)
            {
                return false;
            }

            uint differCount = 0;

            IEnumerator<char> i1 = id1.GetEnumerator();
            IEnumerator<char> i2 = id2.GetEnumerator();

            // This should really be uint, but String.Remove takes int
            // which is stupid, as it should never be < 0
            int pos = 0;
            int differPos = 0;
            while (i1.MoveNext())
            {
                i2.MoveNext();
                if (i1.Current != i2.Current)
                {
                    differCount += 1;
                    differPos = pos;
                }
                pos += 1;
            }
            if (differCount == 1)
            {
                letters = id1.Remove(differPos, 1);
                return true;
            }
            return false;
        }
    }
}

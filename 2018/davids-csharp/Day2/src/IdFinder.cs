using System;
using System.IO;
using System.Collections.Generic;

namespace Day2
{
    public class IDFinder
    {
        public static bool FindSimilar(TextReader inputStream, out string letters)
        {
            letters = null;

            var seenIds = new List<string>();
            while (true)
            {
                var id = inputStream.ReadLine();
                if (id == null)
                {
                    break;
                }
                else if (id.Length < 2)
                {
                    continue;
                }

                foreach (var prevId in seenIds)
                {
                    if (IDDiffer.DifferByOne(prevId, id, out letters))
                    {
                        return true;
                    }
                }
                seenIds.Add(id);
            }

            return false;
        }
    }
}

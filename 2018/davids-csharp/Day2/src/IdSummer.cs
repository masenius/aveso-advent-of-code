using System;
using System.IO;
using System.Collections.Generic;

namespace Day2
{
    public class IDSummer
    {
        public static (bool hasDoubles, bool hasTriples) IDSum(string id)
        {
            var result = (hasDoubles: false, hasTriples: false);
            var seen = new Dictionary<char, uint>();
            foreach (var c in id)
            {
                uint timesSeen;
                if (seen.TryGetValue(c, out timesSeen))
                {
                    seen[c] += 1;
                }
                else
                {
                    seen.Add(c, 1);
                }

            }
            foreach (var entry in seen)
            {
                if (entry.Value == 2)
                {
                    result.hasDoubles = true;
                }
                else if (entry.Value == 3)
                {
                    result.hasTriples = true;
                }
            }

            return result;
        }

        public static uint CalculateSumOfIds(TextReader inputStream)
        {
            uint doubles = 0;
            uint triples = 0;
            while (true)
            {
                var line = inputStream.ReadLine();
                if (line == null)
                {
                    break;
                }
                var rowSum = IDSummer.IDSum(line);
                if (rowSum.hasDoubles)
                {
                    doubles += 1;
                }
                if (rowSum.hasTriples)
                {
                    triples += 1;
                }
            }
            return doubles * triples;
        }
    }
}

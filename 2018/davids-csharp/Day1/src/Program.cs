using System;
using System.IO;
using System.Collections.Generic;

namespace Day1
{
    class Program
    {
        static void Main(string[] args)
        {
            var frequencyFile = args[0];
            int? firstRepeated = null;
            int? endFrequency = null;
            var currentFrequency = 0;
            var reachedFrequencies = new HashSet<int>();

            var inputStream = new StreamReader(frequencyFile);
            var freqReader = new FrequencyReader(inputStream);
            while (true)
            {
                foreach (var freq in freqReader.ReadFrequencies())
                {
                    currentFrequency += freq;

                    if (firstRepeated != null)
                    {
                        continue;
                    }
                    else if (reachedFrequencies.Contains(currentFrequency))
                    {
                        firstRepeated = currentFrequency;

                        // We've already looped at least once and can end immediately
                        if (endFrequency != null)
                        {
                            break;
                        }
                    }
                    else
                    {
                        reachedFrequencies.Add(currentFrequency);
                    }
                }
                if (endFrequency == null)
                {
                    endFrequency = currentFrequency;
                }

                inputStream.Close();

                if (firstRepeated != null)
                {
                    break;
                }

                inputStream = new StreamReader(frequencyFile);
                freqReader.Reset(inputStream);
            }
            System.Console.WriteLine($"End frequency: {endFrequency}");
            System.Console.WriteLine($"First repeated frequency: {firstRepeated}");
        }
    }
}

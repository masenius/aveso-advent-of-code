using System;
using System.IO;

namespace Day1
{
    class Program
    {
        static void Main(string[] args)
        {
            var endFrequency = 0;
            using (var inputStream = new StreamReader(args[0]))
            {
                var freqReader = new FrequencyReader(inputStream);
                foreach (var freq in freqReader.ReadFrequencies())
                {
                    endFrequency += freq;
                }
            }
            System.Console.WriteLine($"Frequency: {endFrequency}");
        }
    }
}

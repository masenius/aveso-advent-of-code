using System;
using System.IO;

namespace Day5
{
    class Program
    {
        static void Main(string[] args)
        {
            var input = new StreamReader(args[0]);
            var polymer = new Polymer(input);

            Console.WriteLine($"Part 1: Remaining length: {polymer.ToString().Length}");

            uint? shortest = null;
            for (var c = 'a'; c <= 'z'; c++)
            {
                // Should really not open the file each time, but...
                input = new StreamReader(args[0]);
                polymer = new Polymer(input, c);
                var length = polymer.ToString().Length;
                if (shortest == null || length < shortest)
                {
                    shortest = (uint) length;
                }
            }

            Console.WriteLine($"Part 2: Shortest possible: {shortest}");
        }
    }
}

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
            polymer.React();

            Console.WriteLine($"Remaining length: {polymer.ToString().Length}");
        }
    }
}

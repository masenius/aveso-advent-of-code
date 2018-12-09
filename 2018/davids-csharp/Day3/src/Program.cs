using System;
using System.IO;

namespace Day3
{
    class Program
    {
        static void Main(string[] args)
        {
            var plotter = new ClothPlotter(new bool[1000,1000]);

            var stream = new StreamReader(args[0]);
            string line;
            while ((line = stream.ReadLine()) != null)
            {
                var claim = ClaimParser.Parse(line);
                plotter.Plot(claim);
            }

            var numberOfOverlapping = plotter.NumberOfOverlapping();
            Console.WriteLine($"Number of overlapping inches: {numberOfOverlapping}");
        }
    }
}

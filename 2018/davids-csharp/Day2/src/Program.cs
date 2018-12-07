using System;
using System.IO;

namespace Day2
{
    class Program
    {
        static void Main(string[] args)
        {
            var idsFile = args[0];
            var inputStream = new StreamReader(idsFile);
            var checkSum = IDSummer.CalculateSumOfIds(inputStream);
            System.Console.WriteLine($"Checksum: {checkSum}");

            inputStream = new StreamReader(idsFile);
            string letters = null;
            if (!IDFinder.FindSimilar(inputStream, out letters))
            {
                System.Console.Error.WriteLine("Error: Could not find similar IDs");
            }
            System.Console.WriteLine($"Common characters: {letters}");
        }
    }
}

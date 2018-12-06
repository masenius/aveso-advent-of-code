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
        }
    }
}

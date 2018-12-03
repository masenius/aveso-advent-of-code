using System;
using System.IO;
using System.Collections.Generic;

namespace Day1
{
    public class FrequencyReader
    {
        private TextReader inputStream;

        public FrequencyReader(TextReader inputStream)
        {
            this.inputStream = inputStream;
        }

        public IEnumerable<int> ReadFrequencies()
        {
            while(true)
            {
                var line = this.inputStream.ReadLine();
                if (line == null)
                {
                    break;
                }
                yield return Convert.ToInt32(line);
            }
        }
    }
}

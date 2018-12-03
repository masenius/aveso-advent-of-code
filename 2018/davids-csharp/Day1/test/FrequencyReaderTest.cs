using System;
using System.Linq;
using System.IO;
using Xunit;

using Day1;

namespace Day1Test
{
    public class FrequencyReaderTest
    {
        [Fact]
        public void TestReadFrequencies()
        {
            var input = "1\n-2\n0\n";
            var reader = new FrequencyReader(new StringReader(input));

            var expected = new int[] {1, -2, 0};
            var actual = reader.ReadFrequencies().ToArray();
            Assert.Equal(expected, actual);
        }
    }
}

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

        [Fact]
        public void TestReset()
        {
            var input = "1\n2\n";
            var reader = new FrequencyReader(new StringReader(input));

            var expected = new int[] {1, 2};
            var actualBeforeReset = reader.ReadFrequencies().ToArray();
            Assert.Equal(expected, actualBeforeReset);

            reader.Reset(new StringReader(input));
            var actualAfterReset = reader.ReadFrequencies().ToArray();
            Assert.Equal(expected, actualAfterReset);
        }
    }
}

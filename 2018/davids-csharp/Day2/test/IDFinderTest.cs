using System;
using System.IO;
using Xunit;

using Day2;

namespace Day2Test
{
    public class IDFinderTest
    {
        [Fact]
        public void TestFindSimilar()
        {
            var input = "abcde\naedcb\nbbaacc\nabfde\n";
            var inputStream = new StringReader(input);

            var expected = "abde";

            string actual = null;
            var foundMatches = IDFinder.FindSimilar(inputStream, out actual);

            Assert.True(foundMatches);
            Assert.Equal(expected, actual);
        }

    }
}

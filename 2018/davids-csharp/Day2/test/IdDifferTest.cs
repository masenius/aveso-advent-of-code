using System;
using System.IO;
using Xunit;

using Day2;

namespace Day2Test
{
    public class IDDifferTest
    {
        [Theory]
        [InlineData("abcde", "abfde", true, "abde")]
        [InlineData("abcde", "abcef", false, null)]
        [InlineData("abcdef", "abcde", false, null)]
        [InlineData("abcde", "abcde", false, null)]
        public void TestDifferByOne(string id1, string id2, bool expectedDiffer, string expectedString)
        {

            string actualString = null;

            var actualDiffer = IDDiffer.DifferByOne(id1, id2, out actualString);
            Assert.Equal(expectedDiffer, actualDiffer);
            Assert.Equal(expectedString, actualString);
        }

    }
}

using System;
using System.IO;
using Xunit;

using Day2;

namespace Day2Test
{
    public class IdSummerTest
    {
        [Theory]
        [InlineData("aabcd", true, false)]
        [InlineData("abcd", false, false)]
        [InlineData("abcddd", false, true)]
        [InlineData("aabcddd", true, true)]
        [InlineData("aabcdddeefff", true, true)]
        [InlineData("aaaa", false, false)]
        public void TestIDSum(string id, bool expectedHasDoubles, bool expectedHasTriples)
        {
            (bool hasDoubles, bool hasTriples) expected = (expectedHasDoubles, expectedHasTriples);
            (bool hasDoubles, bool hasTriples) actual = IDSummer.IDSum(id);

            Assert.Equal(expected, actual);
        }

        [Fact]
        public void TestCalculateSumOfIds()
        {
            var input = "aabcd\nabcd\naabcddd\naaa\n";

            uint expected = 4;
            var actual = IDSummer.CalculateSumOfIds(new StringReader(input));

            Assert.Equal(expected, actual);
        }
    }
}

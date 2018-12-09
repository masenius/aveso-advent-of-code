using System;
using System.Collections.Generic;
using Xunit;

using Day3;

namespace Day3Test
{
    public class ClaimParserTest
    {
        public static IEnumerable<object[]> GetTestdata()
        {
            yield return new object[] { "#1 @ 1,3: 4x4", new Claim(1, 1, 3, 4, 4) };
            yield return new object[] { "#100 @ 20,30: 100x1", new Claim(100, 20, 30, 100, 1) };
        }

        [Theory]
        [MemberData(nameof(GetTestdata))]
        public void TestParseClaim(string input, Claim expected)
        {
            var actual = ClaimParser.Parse(input);
            Assert.Equal(expected.id, actual.id);
            Assert.Equal(expected.x, actual.x);
            Assert.Equal(expected.y, actual.y);
            Assert.Equal(expected.width, actual.width);
            Assert.Equal(expected.height, actual.height);
        }
    }
}

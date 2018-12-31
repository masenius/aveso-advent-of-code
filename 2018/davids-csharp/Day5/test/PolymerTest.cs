using System;
using System.IO;
using Xunit;

using Day5;

namespace Day5Test
{
    public class PolymerTest
    {
        [Fact]
        public void TestReact()
        {
            var input = new StringReader("dabAcCaCBAcCcaDA");
            var polymer = new Polymer(input);

            var expected = "dabCBAcaDA";

            var actual = polymer.ToString();

            Assert.Equal(expected, actual);
        }
    }
}

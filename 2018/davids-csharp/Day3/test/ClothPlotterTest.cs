using System;
using System.Collections.Generic;
using Xunit;

using Day3;

namespace Day3Test
{
    public class ClothPlotterTest
    {

        [Fact]
        public void TestNumberOfOverlapping()
        {
            var canvas = new bool[8,8];
            var plotter = new ClothPlotter(canvas);
            plotter.Plot(new Claim(1, 1, 3, 4, 4));
            plotter.Plot(new Claim(2, 3, 1, 4, 4));
            plotter.Plot(new Claim(3, 5, 5, 2, 2));

            var expected = 4;
            var actual = plotter.NumberOfOverlapping();

            Assert.Equal(expected, actual);
        }
    }
}

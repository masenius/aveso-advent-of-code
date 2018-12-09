using System;
using System.Collections.Generic;

namespace Day3
{
    public class ClothPlotter
    {
        bool[,] canvas;
        HashSet<(uint, uint)> overlapping;

        public ClothPlotter(bool[,] canvas)
        {
            this.canvas = canvas;
            this.overlapping = new HashSet<(uint, uint)>();
        }

        public void Plot(Claim claim)
        {
            for (var x = claim.x; x < claim.x + claim.width; x++)
            {
                for (var y = claim.y; y < claim.y + claim.height; y++)
                {
                    if (this.canvas[x,y])
                    {
                        this.overlapping.Add((x, y));
                    }
                    this.canvas[x,y] = true;
                }
            }
        }

        public int NumberOfOverlapping()
        {
            return this.overlapping.Count;
        }
    }
}

namespace Day3
{
    public struct Claim
    {
        public uint id, x, y, width, height;

        public Claim(uint id, uint x, uint y, uint width, uint height)
        {
            this.id = id;
            this.x = x;
            this.y = y;
            this.width = width;
            this.height = height;
        }
    }
}

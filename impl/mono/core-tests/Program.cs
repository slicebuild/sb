using sb.Core.Tests.Slices;
using sb.Core.Tests.Utils;

namespace sb.Core.Tests
{
    public class Program
    {
        private static void Main()
        {
            new SliceListTests().TracksMissing();
        }
    }
}
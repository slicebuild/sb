using sb.Core.Slices;
using sb.Core.Utils;
using Xunit;

namespace sb.Core.Tests.Slices
{
    public class MissingSliceTests
    {
        [Fact]
        public void ConstructsOk()
        {
            var svi = new SemVerInfo("item-2");
            var slice = new MissingSlice(svi);
            Assert.Equal(slice.RelPath, "item-2");
            Assert.Equal(slice.Info.NameMajor, 2);
        }
    }
}
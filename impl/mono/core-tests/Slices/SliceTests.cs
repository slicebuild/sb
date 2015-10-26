using System.Collections.Generic;
using sb.Core.Slices;
using sb.Core.Utils;
using Xunit;

namespace sb.Core.Tests.Slices
{
    public class SliceTests
    {
        [Fact]
        public void ConstructsOk()
        {
            var sviB = new SemVerInfo("slices-1");
            var svi = new SemVerInfo(sviB.NameSemVer, "item-2");
            var slice = new Slice("root", svi, new List<string> {"OS", "test"});
            Assert.Equal(slice.RelPath, "root");
            Assert.Equal(slice.SemVerInfo.BunchSemVer.Item1, 1);
            Assert.Equal(slice.SemVerInfo.NameSemVer.Item1, 2);
            Assert.Equal(slice.SemVerInfo.Name, "item");
            Assert.Equal(slice.OsList[0], "test");
        }
    }
}
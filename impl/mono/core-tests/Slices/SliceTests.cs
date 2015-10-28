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
            var slice = new Slice("root", svi, new List<string> {""});
            Assert.Equal(slice.RelPath, "root");
            Assert.Equal(slice.Info.BunchSemVer.Item1, 1);
            Assert.Equal(slice.Info.NameSemVer.Item1, 2);
            Assert.Equal(slice.Info.Name, "item");
        }
    }
}
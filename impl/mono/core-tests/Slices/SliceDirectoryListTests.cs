using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using sb.Core.Slices;
using sb.Core.Utils;
using Xunit;

namespace sb.Core.Tests.Slices
{
    public class SliceDirectoryListTests
    {
        //[Fact]todo:do
        public void ScansByVersionMajor()
        {
            var root = Path.Combine(Path.GetTempPath(), new DateTime().ToString("yyyyMMddhhmmss"));
            var paths = new List<string[]>
            {
                new[] {"slices-0", "a", "aaa-1"},
                new[] {"slices-1", "a", "aaa-1"},
                new[] {"slices-1", "b", "bbb-1"},
                new[] {"slices-2", "b", "bbb-1"}
            };

            foreach (var path in paths)
            {
                var f = Path.Combine(root, string.Join(Path.DirectorySeparatorChar.ToString(), path));
                Directory.CreateDirectory(Path.GetDirectoryName(f));
                File.WriteAllText(f, "OS" + Environment.NewLine + "test");
            }

            var sdl = new SliceDirectoryList(root, 1);
            var sliceList = sdl.Scan(new SemVerInfo("test"));

            Assert.Equal(sliceList.Count, 2);
            Directory.Delete(root, true);
        }
    }
}
using System.IO;
using System.Linq;
using sb.Test.Utils;
using sb_bbt.Utils;

namespace sb_bbt.Tests.Fetch
{
    public class TestFetchDeep : Test
    {
        public TestFetchDeep(App app)
            : base(app)
        {
        }

        public override void Run()
        {
            WriteStart();
            Fs.DeleteFolder(App.EnvDir);
            Fs.RunProcess("sb", "fetch", Output);
            var dirCount = new DirectoryInfo(App.EnvDir).GetDirectories().Count();
            var fileCount = new DirectoryInfo(App.EnvDir).GetFiles().Count();
            if (dirCount == 1 && fileCount == 0 && Directory.Exists(App.SlicesDir))
            {
                //todo: download slices and compare dirs
                WriteFinish(true);
                return;
            }
            WriteFinish(false);
        }
    }
}
using System.IO;
using sb.Core.Utils;
using sb.TestBB.Utils;

namespace sb.TestBB.Tests.Fetch
{
    public class TestFetch : Test
    {
        public TestFetch(App app)
            : base(app)
        {
        }

        public override void Run()
        {
            WriteStart();
            FsUtils.DeleteFolder(App.EnvDir);
            Fs.RunProcess("sb", "fetch", Output);
            if (Directory.Exists(App.SlicesDir))
            {
                WriteFinish(true);
                return;
            }
            WriteFinish(false);
        }
    }
}
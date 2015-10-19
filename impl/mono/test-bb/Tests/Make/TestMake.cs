using sb.Core.Utils;
using sb.TestBB.Utils;

namespace sb.TestBB.Tests.Make
{
    public class TestMake : Test
    {
        public TestMake(App app)
            : base(app)
        {
        }

        public override void Run()
        {
            WriteStart();
            FsUtils.DeleteFolderExcept(App.EnvDir, App.SlicesDir);
            Fs.RunProcess("sb", "make", Output);
            if (FsUtils.FileExists(App.EnvDir, "jekyll"))
            {
                WriteFinish(true);
                return;
            }
            WriteFinish(false);
        }
    }
}
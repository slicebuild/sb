using sb.Core.Misc;
using Xunit;

namespace sb.Core.Tests
{
    public class Class11
    {
        [Fact]
        public void PassingTest()
        {
            Assert.Equal(4, new Class1().Test(3));
        }

        [Fact]
        public void FailingTest()
        {
            Assert.Equal(3, new Class1().Test(3));
        }
    }
}

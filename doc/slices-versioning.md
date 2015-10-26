### Rules for internal names of slices for sorting and comparing

1) **All versions adhere to the semver format http://semver.org**

file name can comprise of letters, dashes, underscores, dots and numbers

everything before the first dash + number is considered a name: a-really_yes-really_long-name-2.0

2) **Semver can be in prerelease format**

ruby-2.2.4-alpha.2

ruby-2.2.4-beta.3

ruby-2.2.4-rc.1

3) **In prerelease part there are only 2 places**

alpha

alpha.1

beta.2

rc.3

prerelease names can only be: alpha, beta, rc

4) **Missing parts of semver are inferred**

if there are missing parts in semver, internally it's expanded to the full 5 places format:
major, minor, patch, prerelease, iteration

apache-2 == apache-2.0.0.0.0

slices-1.0.0-alpha == slices-1.0.0-alpha.0

5) **Prelease names internally represented with numbers**

alpha == -3

beta == -2

rc == -1

6) **All slices are kept in a directory with semver name** 

for example: slices-1.0.0, rules from above apply to semver processing

the whole slices directory is referred to as 'bunch' for this particular version of slices

7) **There can be more than one directory (bunch) of slices in the same location**

.sb/slices/slices-1.0.0

.sb/slices/my-slices-1.0.0

.sb/slices/slices-2.1.1

.sb/slices/other-slices-3.0.1-alpha.1

8) **Major parts of the tool version and the slice bunches correspond to one another**

when the tool looks for the slices, it selects the bunches that have MAJOR version equal to the MAJOR version of the tool:

sb version 2* == slices-2*

9) **Internal name of a slice is of 3 parts**

* file name in lower case

* file semver

* bunch semver  

ex: mysql-1.2.3 from slices-1.0.0 == mysql | 1.2.3.0.0 | 1.0.0.0.0

10) **Internally the list of slices is sorted**

sorting is done by comparing:

(first) bunch semver DESC

(then) file semver DESC

(then) file name ASC or DESC

*the bunch name is not considered*

examples of a sorted list of slices:

1) example of one bunch of slices in a folder slices-1.2.3

1.2.3.0.0 | 4.1.2.0.0 | mysql

1.2.3.0.0 | 2.0.0.0.0 | apache

1.2.3.0.0 | 1.0.0.0.0 | apache

2) example of two bunches of slices: slices-1.2.3 and slices-1.2.4-rc.1

1.2.4.-1.1 | 4.1.2.0.0 | mysql

1.2.4.-1.1 | 2.0.0.0.0 | apache

1.2.3.0.0 | 4.1.2.0.0 | mysql

1.2.3.0.0 | 2.0.0.0.0 | apache

1.2.3.0.0 | 1.0.0.0.0 | apache

11) **Two slices with the same name, version and bunch version are not allowed**

slices-1.0.0/a/apache CONFLICTS WITH slices-1.0.0/a/a/a/apache 

not allowed even across bunches (since the bunch name is not considered):

myslices-1.2.2/m/mysql-4.1 CONFLICTS WITH slices-1.2.2/m/m/mysql-4.1.0

### Rules for selecting slices

1) **If no version requested - then the latest wins**

debian == topmost in the sorted list with the name 'debian' (considering the bunch version)

2) **If version provided - the highest starting at this version wins**

mysql-4 == latest equal or above 4.0.0.0.0 with the name 'mysql' (considering the bunch version)

ruby-2.1.2 == latest equal or above 2.1.2.0.0 with the name 'ruby' (considering the bunch version)

*it means that internally there should be understanding how the slice name looked before been expanded to the 3 parts format.*

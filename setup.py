from distutils.core import setup

context = {}
execfile('sala', context, context)

setup(
    name='sala',
    version=context['version'],
    author='Petri Lehtinen',
    author_email='petri@digip.org',
    url='http://www.digip.org/sala/',
    description='Encrypted plaintext password store',
    long_description=open('README.rst').read(),
    license='MIT',
    scripts=['sala'],
)

require_relative 'xunit'

class WasRun < TestCase
  attr_reader :log

  def set_up
    @log = 'set_up '
  end

  def tear_down
    @log += 'tear_down '
  end

  def test_method
    @log += 'test_method '
  end

  def test_broken_method
    raise
  end
end

class TestCaseTest < TestCase
  def set_up
    @result = TestResult.new
  end

  def test_template_method
    test = WasRun.new('test_method')
    test.run @result
    assert! test.log == 'set_up test_method tear_down '
  end

  def test_result
    test = WasRun.new('test_method')
    test.run @result
    assert! @result.summary == '1 run, 0 failed'
  end

  def test_failed_result
    test = WasRun.new('test_broken_method')
    test.run @result
    assert! @result.summary == '1 run, 1 failed'
  end

  def test_failed_result_formatting
    @result.test_started
    @result.test_failed
    assert! @result.summary == '1 run, 1 failed'
  end

  def test_suite
    suite = TestSuite.new
    suite.add WasRun.new('test_method')
    suite.add WasRun.new('test_broken_method')
    suite.run @result
    assert! @result.summary == '2 run, 1 failed'
  end
end

suite = TestSuite.new
suite.add TestCaseTest.new('test_template_method')
suite.add TestCaseTest.new('test_result')
suite.add TestCaseTest.new('test_failed_result_formatting')
suite.add TestCaseTest.new('test_failed_result')
suite.add TestCaseTest.new('test_suite')
result = TestResult.new
suite.run result
puts result.summary

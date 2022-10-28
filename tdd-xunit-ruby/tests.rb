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
  def test_template_method
    test = WasRun.new('test_method')
    test.run
    assert! test.log == 'set_up test_method tear_down '
  end

  def test_result
    test = WasRun.new('test_method')
    result = test.run
    assert! result.summary == '1 run, 0 failed'
  end

  def test_failed_result
    test = WasRun.new('test_broken_method')
    result = test.run
    assert! result.summary == '1 run, 1 failed'
  end

  def test_failed_result_formatting
    result = TestResult.new
    result.test_started
    result.test_failed
    assert! result.summary == '1 run, 1 failed'
  end
end

puts TestCaseTest.new('test_template_method').run.summary
puts TestCaseTest.new('test_result').run.summary
puts TestCaseTest.new('test_failed_result_formatting').run.summary
puts TestCaseTest.new('test_failed_result').run.summary

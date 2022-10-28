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
end

TestCaseTest.new('test_template_method').run
TestCaseTest.new('test_result').run
TestCaseTest.new('test_failed_result').run
